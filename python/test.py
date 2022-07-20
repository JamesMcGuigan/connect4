#!/usr/bin/env python3

import connectx
from kaggle_environments import structify
from kaggle_environments.envs.connectx.connectx import is_win, play
from connectx import Observation, Configuration

from main import connectx_agent


def print_board(obs, conf):
    for row in range(conf.rows):
        print(obs.board[ row * conf.columns : (row+1) * conf.columns ])


def play_game(obs, conf, agents):
    while True:
        agent  = agents[obs.mark-1]
        action = agent(obs, conf, verbose=False, TEST_SUBMISSION=False)
        if obs.board[action] != 0:
            print(f"ERROR: {obs.step:2d} | agent({obs.mark}) = {action}")
            break

        play(obs.board, action, obs.mark, conf)
        print(f"{obs.step:2d} | agent({obs.mark}) = {action}")

        if is_win(obs.board, action, obs.mark, conf, has_played=True):
            print(f"WIN: Player {obs.mark}")
            break
        elif obs.step >= len(obs.board)-1:  # >= 41
            print(f"DRAW: obs.step = {obs.step}")
            break
        else:
            obs.mark = 2 if obs.mark == 1 else 1
            obs.step += 1

    print_board(obs, conf)


# Recompile: poetry run maturin develop
if __name__ == '__main__':
    obs    = structify({ 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]})
    conf   = structify({ 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' })
    rust_obs  = Observation(**obs)
    rust_conf = Configuration(**conf)

    agents = [ connectx.agent_random, connectx.agent_modulo ]
    print("connectx.agent_random(rust_obs, rust_conf) =", connectx.agent_random(rust_obs, rust_conf))
    print("connectx.agent_modulo(rust_obs, rust_conf) =", connectx.agent_modulo(rust_obs, rust_conf))
    play_game(obs, conf, agents)
