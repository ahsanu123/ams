use sea_orm::ActiveModelBehavior;

pub trait ToActiveModel<AM>
where
    AM: ActiveModelBehavior,
{
    fn to_active_without_id(&self) -> AM;
    fn to_active_with_id(&self) -> AM;
}
