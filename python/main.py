#!/usr/bin/env python3
TEST_SUBMISSION = True

# Source: https://www.kaggle.com/c/halite/discussion/177686
# BUGFIX: Kaggle Submission Environment os.getcwd() == "/kaggle/working/"
import os
if os.environ.get('GFOOTBALL_DATA_DIR', ''):
    os.chdir('/kaggle_simulations/agent/')

# noinspection PyUnresolvedReferences
import maturin_kaggle
import time
from kaggle_environments.envs.connectx.connectx import is_win


# DOCS: https://www.kaggle.com/competitions/connectx/overview/environment-rules
# DOCS: https://www.kaggle.com/c/halite/discussion/177686
def agent(obs, conf):
    # obs  = { 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}
    # conf = { 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' }
    time_start = time.perf_counter()

    # noinspection PyUnresolvedReferences
    action = maturin_kaggle.random_move(conf.columns)

    time_taken = time.perf_counter() - time_start
    print(f"action = {action} | {time_taken:.3f}s")
    if obs.step <= 1:
        print(" obs  =", obs)
        print(" conf =", conf)
    if TEST_SUBMISSION:
        if is_win(obs.board, action, obs.mark, conf, has_played=False) or obs.step >= 40:
            print(" TEST_SUBMISSION | shortcircuit to prevent kaggle submission")
            print(" obs  =", obs)
            print(" conf =", conf)
            raise TimeoutError  # shortcircuit to prevent kaggle submission

    return action
