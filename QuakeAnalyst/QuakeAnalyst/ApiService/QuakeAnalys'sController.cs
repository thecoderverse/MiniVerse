using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using QuakeAnalyst.Controllers;
using QuakeAnalyst.Repo;
using System.Linq;

namespace QuakeAnalyst.ApiService
{
    [Route("api/[controller]")]
    [ApiController]
    public class QuakeAnalysisController : ControllerBase
    {
        private readonly ILogger<HomeController> _logger;
        private readonly OrhanAydogduApiHandler _apiHandler;
        [HttpPost]
        public async Task<IActionResult> DailyDensity()
        {
            List<Earthquake> earthquakes = await _apiHandler.GetEarthquakes(new DateTime(1990,1,1), DateTime.Now);
            var density = from q in earthquakes group q by q.Date.Hour into g select g.Average(x => x.Magnitude);
            return (IActionResult)density.ToList();
        }
    }
}