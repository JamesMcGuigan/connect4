import connectx  # pyproject.toml | [project] name = "connectx"
from middleware import middleware_agent

### Submission Config
submission_agent = middleware_agent(
    connectx.agent_random,   # import rust: agents/agent_random.rs
    verbose=True,            # print moves and board to stdout
    TEST_SUBMISSION=False    # throw exception before victory == fail leaderboard submission validation episode
)
