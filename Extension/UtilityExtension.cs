namespace AMS.Extension;

using System.ComponentModel;
using System.Runtime.CompilerServices;

public static class UtilityExtension
{
    // object To Dictionary Copied from :https://gist.github.com/jarrettmeyer/798667/a87f9bcac2ec68541511f17da3c244c0e05bdc49
    private static void ThrowExceptionWhenSourceArgumentIsNull()
    {
        throw new NullReferenceException(
            "Unable to convert anonymous object to a dictionary. The source anonymous object is null."
        );
    }

    private static bool IsOfType<T>(object value)
    {
        return value is T;
    }

    public static IDictionary<string, T> ToDictionary<T>(this T source, bool removeId = false)
    {
        if (source == null)
            ThrowExceptionWhenSourceArgumentIsNull();

        var dictionary = new Dictionary<string, T>();
        foreach (PropertyDescriptor property in TypeDescriptor.GetProperties(source))
        {
            object value = property.GetValue(source);
            if (IsOfType<T>(value))
            {
                Console.WriteLine(property.Name);
                dictionary.Add(property.Name, (T)value);
            }
        }
        return dictionary;
    }

    //  reference https://stackoverflow.com/a/16295702/19270838
    public static void ShowCurrentPosition(
        string message = "",
        [CallerFilePath] string file = "",
        [CallerMemberName] string member = "",
        [CallerLineNumber] int line = 0
    )
    {
        Console.WriteLine("-------------------------");
        Console.WriteLine($"{Path.GetFileName(file)}_{member}(line: {line}) -- \n{message}");
        Console.WriteLine("-------------------------");
    }

    // reference: https://stackoverflow.com/a/75958935/19270838
    // official docs: https://learn.microsoft.com/en-us/dotnet/api/system.runtime.compilerservices.callerargumentexpressionattribute.-ctor?view=net-7.0#system-runtime-compilerservices-callerargumentexpressionattribute-ctor(system-string)
    public static string FullNameof(
        string propertyName,
        [CallerArgumentExpression(nameof(propertyName))] string fullNameof = ""
    )
    {
        return fullNameof.Substring(7, fullNameof.Length - 8);
    }

    public static string GetLastWordAfterDot(string dotSeparatedString)
    {
        return dotSeparatedString.Split(".").Last();
    }
}
