using FluentMigrator;
using FluentMigrator.Runner;
using FluentMigrator.Runner.BatchParser;
using FluentMigrator.Runner.Generators;
using FluentMigrator.Runner.Generators.SQLite;
using FluentMigrator.Runner.Initialization;
using FluentMigrator.Runner.Processors;
using FluentMigrator.Runner.Processors.SQLite;
using Microsoft.Extensions.Options;

namespace AMS.Extension;

public static class FluentMigratorExtension
{
    public static IMigrationRunnerBuilder AddSQLiteWithCompatibilityMode(
        this IMigrationRunnerBuilder builder,
        bool binaryGuid = false,
        bool useStrictTables = false,
        CompatibilityMode compatibilityMode = CompatibilityMode.STRICT
    )
    {
        builder
            .Services.AddTransient<SQLiteBatchParser>()
            .AddScoped<SQLiteDbFactory>()
            .AddScoped<SQLiteProcessor>(sp =>
            {
                var factory = sp.GetService<SQLiteDbFactory>();
                var logger = sp.GetService<ILogger<SQLiteProcessor>>();
                var options = sp.GetService<IOptionsSnapshot<ProcessorOptions>>();
                var connectionStringAccessor = sp.GetService<IConnectionStringAccessor>();
                var sqliteQuoter = new SQLiteQuoter(false);
                return new SQLiteProcessor(
                    factory,
                    sp.GetService<SQLiteGenerator>(),
                    logger,
                    options,
                    connectionStringAccessor,
                    sp,
                    sqliteQuoter
                );
            })
            .AddScoped<ISQLiteTypeMap>(sp => new SQLiteTypeMap(useStrictTables))
            .AddScoped<IMigrationProcessor>(sp => sp.GetRequiredService<SQLiteProcessor>())
            .AddScoped(sp =>
            {
                var typeMap = sp.GetRequiredService<ISQLiteTypeMap>();
                return new SQLiteGenerator(
                    new SQLiteQuoter(binaryGuid),
                    typeMap,
                    new OptionsWrapper<GeneratorOptions>(
                        new GeneratorOptions { CompatibilityMode = compatibilityMode }
                    )
                );
            })
            .AddScoped<IMigrationGenerator>(sp => sp.GetRequiredService<SQLiteGenerator>());

        return builder;
    }
}
