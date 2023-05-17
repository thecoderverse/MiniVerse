using System.ComponentModel.DataAnnotations;
using System.Text.Json.Serialization;

namespace AgileTips.Api.Models;
public class Tip
{
    public int Id { get; set; }
    [Required(ErrorMessage = "Tip title can't be null or empty")]
    public string Title { get; set; }
    [Required(ErrorMessage = "Tip description can't be null or empty")]
    public string Description { get; set; }
    [JsonIgnore]
    public Scope Scope { get; set; }
    public int ScopeId {get;set;}
}
