#!/usr/bin/env python3

import maturin_kaggle
from kaggle_environments import structify
from kaggle_environments.envs.connectx.connectx import is_win, play

from main import maturin_kaggle_agent


def play_game(obs, conf, agents):
    while True:
        agent  = agents[obs.mark-1]
        action = agent(obs, conf, verbose=False, TEST_SUBMISSION=False)
        play(obs.board, action, obs.mark, conf)
        print(f"agent({obs.mark}) = {action}")
        if obs.step >= 42:
            print(f"DRAW: obs.step = {obs.step}")
            break
        elif is_win(obs.board, action, obs.mark, conf, has_played=True):
            print(f"WIN: p{obs.mark} | obs.step = {obs.step}")
            for row in range(conf.rows):
                print(obs.board[ row * conf.columns : (row+1) * conf.columns ])
            break
        else:
            obs.mark = 2 if obs.mark == 1 else 1
            obs.step += 1

# Recompile: poetry run maturin develop
if __name__ == '__main__':
    obs    = structify({ 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]})
    conf   = structify({ 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' })
    agents = [ maturin_kaggle_agent, maturin_kaggle_agent ]

    print("maturin_kaggle.random_move(conf.columns) =", maturin_kaggle.random_move(conf.columns))
    play_game(obs, conf, agents)

