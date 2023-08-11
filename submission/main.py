#!/usr/bin/env python3
# Kaggle Agent Entrypoint File
# Submission Config in: ./agent.py

# BUGFIX: Kaggle Submission Environment os.getcwd() == "/kaggle/working/"
# Source: https://www.kaggle.com/c/halite/discussion/177686
# DOCS:   https://www.kaggle.com/competitions/connectx/overview/environment-rules
import os
if os.environ.get('GFOOTBALL_DATA_DIR', ''):
    os.chdir('/kaggle_simulations/agent/')

from agent import submission_agent
from kaggle_environments import structify


### Kaggle Agent Function
def kaggle_agent(obs, conf):
    if '__raw_path__' in conf: del conf['__raw_path__']
    action = submission_agent(obs, conf)  # agent.py
    return action


if __name__ == '__main__':
    obs  = structify({ 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]})
    conf = structify({ 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' })
    kaggle_agent(obs, conf)
