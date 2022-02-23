use std::collections::HashSet;

use crate::{
    models::Channel, permissions::PermissionCalculator, Override, Permission, PermissionValue,
    Permissions, Perms, Result, DEFAULT_PERMISSION_DIRECT_MESSAGE,
    DEFAULT_PERMISSION_SAVED_MESSAGES,
};

impl PermissionCalculator<'_> {
    /// Calculate the permissions from our perspective to the given server or channel
    ///
    /// Refer to permission_hierarchy.svg for more information
    pub async fn calc(&mut self, db: &crate::Database) -> Result<Perms> {
        let value = if self.channel.has() {
            calculate_channel_permission(self, db).await?
        } else if self.server.has() {
            calculate_server_permission(self, db).await?
        } else {
            panic!("Expected `PermissionCalculator.(user|server) to exist.");
        }
        .into();

        self.cached_permission = Some(value);
        Ok(Permissions([value]))
    }
}

/// Internal helper function for calculating server permission
async fn calculate_server_permission(
    data: &mut PermissionCalculator<'_>,
    db: &crate::Database,
) -> Result<PermissionValue> {
    let server = data.server.get().unwrap();

    // 1. Check if owner.
    if data.perspective.id == server.owner {
        return Ok((Permission::GrantAllSafe as u64).into());
    }

    // 2. Fetch member.
    if !data.member.has() {
        data.member
            .set(db.fetch_member(&server.id, &data.perspective.id).await?);
    }

    let member = data.member.get().expect("Member should be present by now.");

    // 3. Apply allows from default_permissions.
    let mut permissions: PermissionValue = server.default_permissions.into();

    // 4. Resolve each role in order.
    let member_roles: HashSet<&String> = if let Some(roles) = member.roles.as_ref() {
        roles.iter().collect()
    } else {
        HashSet::new()
    };

    if !member_roles.is_empty() {
        let mut roles = server
            .roles
            .iter()
            .filter(|(id, _)| member_roles.contains(id))
            .map(|(_, role)| {
                let v: Override = role.permissions.into();
                (v.rank(role.rank), v)
            })
            .collect::<Vec<(i64, Override)>>();

        roles.sort_by(|a, b| b.0.cmp(&a.0));

        // 5. Apply allows and denies from roles.
        for (_, v) in roles {
            permissions.apply(v);
        }
    }

    Ok(permissions)
}

/// Internal helper function for calculating channel permission
async fn calculate_channel_permission(
    data: &mut PermissionCalculator<'_>,
    db: &crate::Database,
) -> Result<PermissionValue> {
    // Pre-calculate server permissions if applicable.
    // We do this to satisfy the borrow checker.
    let server_id = match data.channel.get().unwrap() {
        Channel::TextChannel { server, .. } | Channel::VoiceChannel { server, .. } => Some(server),
        _ => None,
    };

    let mut permissions = if let Some(server) = server_id {
        if !data.server.has() {
            data.server.set(db.fetch_server(server).await?);
        }

        calculate_server_permission(data, db).await?
    } else {
        0_u64.into()
    };

    // Borrow the channel now and continue as normal.
    let channel = data.channel.get().unwrap();

    // 1. Check channel type.
    let value: PermissionValue = match channel {
        Channel::SavedMessages { .. } => (*DEFAULT_PERMISSION_SAVED_MESSAGES).into(),
        Channel::DirectMessage { .. } => (*DEFAULT_PERMISSION_DIRECT_MESSAGE).into(),
        Channel::Group {
            owner, permissions, ..
        } => {
            // 2. Check if user is owner.
            if &data.perspective.id == owner {
                (Permission::GrantAllSafe as u64).into()
            } else {
                // 3. Pull out group permissions.
                permissions
                    .map(|x| x as u64)
                    .unwrap_or(*DEFAULT_PERMISSION_DIRECT_MESSAGE)
                    .into()
            }
        }
        Channel::TextChannel {
            default_permissions,
            role_permissions,
            ..
        }
        | Channel::VoiceChannel {
            default_permissions,
            role_permissions,
            ..
        } => {
            // 2. If server owner, just grant all permissions.
            if let Some(member) = data.member.get() {
                // 3. Apply default allows and denies for channel.
                if let Some(default) = default_permissions {
                    permissions.apply((*default).into());
                }

                // 4. Resolve each role in order.
                let member_roles: HashSet<&String> = if let Some(roles) = member.roles.as_ref() {
                    roles.iter().collect()
                } else {
                    HashSet::new()
                };

                if !member_roles.is_empty() {
                    let server = data.server.get().unwrap();
                    let mut roles = role_permissions
                        .iter()
                        .filter(|(id, _)| member_roles.contains(id))
                        .map(|(id, permission)| {
                            let role = server.roles.get(id).expect("Role on server object");
                            let v: Override = (*permission).into();
                            (v.rank(role.rank), v)
                        })
                        .collect::<Vec<(i64, Override)>>();

                    roles.sort_by(|a, b| b.0.cmp(&a.0));

                    // 5. Apply allows and denies from roles.
                    for (_, v) in roles {
                        permissions.apply(v);
                    }
                }

                permissions
            } else {
                (Permission::GrantAllSafe as u64).into()
            }
        }
    };

    Ok(value)
}
