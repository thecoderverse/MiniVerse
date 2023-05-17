using AgileTips.Api.Data;
using AgileTips.Api.Models;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;

namespace AgileTips.Api.Controllers;
[Route("api/[controller]")]
[ApiController]
public class TipsController
: ControllerBase
{
    private readonly ApiDbContext _dbContext;

    public TipsController(IConfiguration config)
    {
        _dbContext = new ApiDbContext(config);
    }

    [HttpPost]
    public IActionResult Post([FromBody] Tip tip)
    {
        if (tip == null)
        {
            return NoContent();
        }
        else
        {
            _dbContext.Tips.Add(tip);
            _dbContext.SaveChanges();

            return StatusCode(StatusCodes.Status201Created);
        }
    }

    [HttpGet("List")]
    public IActionResult GetByScope(int scopeId)
    {
        var tips = _dbContext.Tips.Include(t=>t.Scope).Where(c => c.Scope.Id == scopeId);
        if (tips == null)
        {
            return NotFound();
        }
        return Ok(tips);
    }

    [HttpGet("Daily")]
    public IActionResult GetRandom()
    {
        var result = _dbContext
                    .Tips
                    .Where(t => t.Id == 1);

        if (result == null)
        {
            return NotFound();
        }
        return Ok(result);
    }
}
