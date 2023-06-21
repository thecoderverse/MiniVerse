using Microsoft.AspNetCore.Mvc;
using QuakeAnalyst.ApiService;
using QuakeAnalyst.MvcModels;
using System.Diagnostics;

namespace QuakeAnalyst.Controllers
{
    public class HomeController : Controller
    {
        private readonly ILogger<HomeController> _logger;
        private readonly OrhanAydogduApiHandler _apiHandler;

        public HomeController(ILogger<HomeController> logger, OrhanAydogduApiHandler ApiHandler)
        {
            _logger = logger;
            _apiHandler = ApiHandler;
        }

        public IActionResult Index()
        {
            return View();
        }

        public IActionResult Privacy()
        {
            return View();
        }
        public async Task<IActionResult> GeoLocations()
        {
            var model = new GeoLocationsModel();
            model.GeoLocations = await _apiHandler.GetCities();
            return View(model);
        }

        public ActionResult EarthQuakes()
        {
            var model = new EarthquakesModel();
            return View(model);
        }

        [HttpGet("fromDay")]
        public async Task<IActionResult> EarthQuakes(string fromDay, string toDay)
        {
            var model = new EarthquakesModel();
            var from = DateTime.Parse(fromDay);
            var to = DateTime.Parse(toDay);

            model.Earthquakes = await _apiHandler.GetEarthquakes(from, to);
            return View(model);
        }

        [ResponseCache(Duration = 0, Location = ResponseCacheLocation.None, NoStore = true)]
        public IActionResult Error()
        {
            return View(new ErrorViewModel { RequestId = Activity.Current?.Id ?? HttpContext.TraceIdentifier });
        }
    }
}