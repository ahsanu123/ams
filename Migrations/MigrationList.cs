using AMS.Model;
using FluentMigrator;

namespace AMS.InternalMigration;

public class ModelMigrationList : MigrationBase
{
    private List<Type> listModel = new List<Type>
    {
        typeof(Role),
        typeof(User),
        typeof(RoleClaim),
        typeof(UserClaim),
        typeof(UserLogin),
        typeof(UserRole),
        typeof(UserToken),
        typeof(AmpasModel),
        typeof(AmpasPrice),
        typeof(Production),
        typeof(CampaignModel),
    };

    public static List<string> listExcludedType = new List<string> { nameof(System.Collections) };

    public void MigrationDown(Migration migration)
    {
        this.listModel.ForEach((type) => migration.DeleteTableIfExists(type));
    }

    public void MigrationUp(Migration migration)
    {
        this.listModel.ForEach((type) => migration.ConvertModelToMigration(type, listExcludedType));
    }

    public void GenerateForeignKey(Migration migration)
    {
        this.listModel.ForEach((type) => migration.GenerateForeignKey(type));
    }
}
