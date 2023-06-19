using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;

namespace QuakeAnalyst.Repo
{
    public class GeoLocation
    {
        [JsonProperty("city")]
        public string Name { get; set; }
        [JsonProperty("cityCode")]
        public int CityCode { get; set; }
        [JsonProperty("population")]
        public int? Populations { get; set; }
    }
}
