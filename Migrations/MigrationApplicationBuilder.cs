using FluentMigrator;
using FluentMigrator.Runner;

namespace AMS.InternalMigration;

public static class MigrationApplicationBuilder
{
    public const int MIGRATION_VERSION = 60;

    public const string MIGRATION_DESCRIPTION =
        $"Learning Authentication and Authorization From stratch";

    public static bool UpdateForeignKey = false;

    public static IApplicationBuilder UseFluentMigrator(this IApplicationBuilder appBuilder)
    {
        using var scope = appBuilder.ApplicationServices.CreateScope();

        var runner = scope.ServiceProvider.GetService<IMigrationRunner>();
        var versionLoader = scope.ServiceProvider.GetService<IVersionLoader>();

        if (runner == null)
            throw new ArgumentNullException("Fluent Migrator runner was Null!!");
        if (versionLoader == null)
            throw new ArgumentNullException("Fluent Migrator version loader was Null!!");

        runner.ListMigrations();

        if (MigrationApplicationBuilder.MIGRATION_VERSION > versionLoader.VersionInfo.Latest())
        {
            runner.Down(new MainMigration());

            runner.MigrateUp(MigrationApplicationBuilder.MIGRATION_VERSION);

            // this is for update foreign key
            // after table is established in database
            // MigrationExtension.UpdateForeignKey = true;

            MigrationApplicationBuilder.UpdateForeignKey = true;
            runner.Up(new MainMigration());
        }

        return appBuilder;
    }

    public static Migration DeleteTableIfExists(this Migration migration, Type type)
    {
        migration.Delete.Table(type.Name).IfExists();

        return migration;
    }
}
