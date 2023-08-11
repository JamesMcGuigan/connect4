#!/usr/bin/env python3

import connectx
from kaggle_environments import structify
from kaggle_environments.envs.connectx.connectx import is_win, play
from middleware import middleware_agent


def print_board(obs, conf):
    for row in range(conf.rows):
        print(obs.board[ row * conf.columns : (row+1) * conf.columns ])


def play_game(obs, conf, agents):
    while True:
        agent  = agents[obs.mark-1]
        action = agent(obs, conf)
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

    ### Config
    agents = [
        middleware_agent(connectx.agent_random, verbose=False, TEST_SUBMISSION=False),
        middleware_agent(connectx.agent_modulo, verbose=False, TEST_SUBMISSION=False),
    ]
    obs  = structify({ 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]})
    conf = structify({ 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' })

    ### Test
    print("middleware_agent( connectx.agent_random, verbose=False, TEST_SUBMISSION=False ) =", agents[0](obs, conf))
    print("middleware_agent( connectx.agent_modulo, verbose=False, TEST_SUBMISSION=False ) =", agents[1](obs, conf))
    play_game(obs, conf, agents)
