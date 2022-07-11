#!/usr/bin/env python3

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
def maturin_kaggle_agent(obs, conf, verbose=True, TEST_SUBMISSION=True):
    # obs  = { 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}
    # conf = { 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' }
    time_start = time.perf_counter()

    # noinspection PyUnresolvedReferences
    # action = maturin_kaggle.random_move(obs, conf)
    action = maturin_kaggle.random_move_args(
        obs.step,
        obs.mark,
        obs.board,
        obs.remainingOverageTime,
        conf.columns,
        conf.rows,
        conf.inarow,
        conf.timeout,
        conf.actTimeout,
        conf.agentTimeout,
        conf.episodeSteps,
        conf.runTimeout,
    )

    time_taken = time.perf_counter() - time_start
    if verbose: print(f" action = {action} | {time_taken:.3f}s")
    if obs.step <= 1:
        if verbose: print(" obs  =", obs)
        if verbose: print(" conf =", conf)
    if TEST_SUBMISSION:
        if is_win(obs.board, action, obs.mark, conf, has_played=False) or obs.step >= 40:
            print(" TEST_SUBMISSION | shortcircuit to prevent kaggle submission")
            print(" obs  =", obs)
            print(" conf =", conf)
            raise TimeoutError  # shortcircuit to prevent kaggle submission

    return action
