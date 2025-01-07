using System.Reflection;
using FluentMigrator;

namespace AMS.InternalMigration;

[Migration(
    MigrationApplicationBuilder.MIGRATION_VERSION,
    MigrationApplicationBuilder.MIGRATION_DESCRIPTION
)]
public class MainMigration : Migration
{
    IEnumerable<Type> listMigration = Assembly
        .GetAssembly(typeof(MigrationBase))
        .GetTypes()
        .Where(type =>
            type.IsClass && !type.IsAbstract && typeof(MigrationBase).IsAssignableFrom(type)
        );

    List<MigrationBase> GetMigrationBaseInheritedClass()
    {
        List<MigrationBase> list = new List<MigrationBase>();

        foreach (var item in listMigration)
        {
            // if (System.Attribute.GetCustomAttributes(item).Length == 0)
            // {
            list.Add((MigrationBase)Activator.CreateInstance(item));
            // }
        }
        return list;
    }

    public override void Down()
    {
        foreach (var item in GetMigrationBaseInheritedClass())
        {
            item.MigrationDown(this);
        }
    }

    public override void Up()
    {
        if (!MigrationApplicationBuilder.UpdateForeignKey)
        {
            foreach (var item in GetMigrationBaseInheritedClass())
            {
                item.MigrationUp(this);
            }
        }

        if (MigrationApplicationBuilder.UpdateForeignKey)
        {
            foreach (var item in GetMigrationBaseInheritedClass())
            {
                item.GenerateForeignKey(this);
            }
        }
    }
}
