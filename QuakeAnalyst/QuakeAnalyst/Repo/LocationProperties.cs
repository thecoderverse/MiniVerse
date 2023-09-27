using Newtonsoft.Json;

namespace QuakeAnalyst.Repo
{
    public class LocationProperties
    {
        [JsonProperty("ClosestCity")]
        public City ClosestCity { get; set; }
        [JsonProperty("epiCenter")]
        public City EpiCenter { get; set; }
    }
}
