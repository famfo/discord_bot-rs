use serenity::client::Context;
use serenity::model::guild::{Member, Role, Permission};

pub async fn checkPerm(ctx: &Context, mut member_orig:Member, mut member_targ:Member) -> bool
{
	return getHighestRole(member_orig).position > getHighestRole(member_targ).position;
}

pub async fn getHighestRole(ctx: &Context, mut member:Member) -> Role
{
	return member_orig.roles(&ctx.cache).await.unwrap().sort_by(|a,b| b.position.cmp(&a.position))[0]
}

pub async fn checkPerm(ctx: &Context, mut member:Member, mut perm:Permission) -> bool
{
	return member.roles(&ctx.cache).await.unwrap().iter().any(|r| r.permissions.contains(perm));
}