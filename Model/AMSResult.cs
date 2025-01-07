namespace AMS.Model;

public class AMSResult
{
    public bool Success { get; set; }
    public IEnumerable<string>? Message { get; set; }
}
