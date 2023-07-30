import time
import connectx
# from connectx.connectx import Observation, Configuration
from kaggle_environments.envs.connectx.connectx import is_win


# DOCS: https://www.kaggle.com/competitions/connectx/overview/environment-rules
# DOCS: https://www.kaggle.com/c/halite/discussion/177686
def middleware_agent(rust_agent, verbose=True, TEST_SUBMISSION=True):
    def middleware(obs, conf):
        # obs  = { 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}
        # conf = { 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' }
        time_start = time.perf_counter()

        rust_obs  = connectx.Observation(**obs)      # rust = structs/kaggle.rs
        rust_conf = connectx.Configuration(**conf)   # rust = structs/kaggle.rs
        action    = rust_agent(rust_obs, rust_conf)

        time_taken = time.perf_counter() - time_start
        if verbose: print(f" action = {action} | {time_taken:.3f}s")
        if obs.step <= 1:
            if verbose: print(" obs  =", obs)
            if verbose: print(" conf =", conf)
        if TEST_SUBMISSION:
            if is_win(obs.board, action, obs.mark, conf, has_played=False) or obs.step >= 40:
                print(" TEST_SUBMISSION | shortcircuit before victory to prevent kaggle submission")
                print(" obs  =", obs)
                print(" conf =", conf)
                raise TimeoutError  # shortcircuit to prevent kaggle submission

        return action

    return middleware

