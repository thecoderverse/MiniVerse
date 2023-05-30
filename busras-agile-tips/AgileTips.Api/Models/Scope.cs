using System.ComponentModel.DataAnnotations;

namespace AgileTips.Api.Models;
public class Scope
{
    public int Id { get; set; }
    [Required(ErrorMessage = "Scope name can't be null or empty")]
    public string Name { get; set; }
    public ICollection<Tip> Tips { get; set; }
}
