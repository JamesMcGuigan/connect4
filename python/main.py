#!/usr/bin/env python3

# Source: https://www.kaggle.com/c/halite/discussion/177686
# BUGFIX: Kaggle Submission Environment os.getcwd() == "/kaggle/working/"
import os
if os.environ.get('GFOOTBALL_DATA_DIR', ''):
    os.chdir('/kaggle_simulations/agent/')

import maturin_kaggle

if __name__ == '__main__':
    print("maturin_kaggle.random_move(7) =", maturin_kaggle.random_move(7))

# DOCS: https://www.kaggle.com/competitions/connectx/overview/environment-rules
# DOCS: https://www.kaggle.com/c/halite/discussion/177686
def agent(observation, configuration):
    columns = configuration.columns  # Number of Columns on the Board.
    rows    = configuration.rows     # Number of Rows on the Board.
    inarow  = configuration.inarow   # Number of Checkers "in a row" needed to win.
    board   = observation.board      # The current serialized Board (rows x columns).
    mark    = observation.mark       # Which player the agent is playing as (1 or 2).
    action  = maturin_kaggle.random_move(columns)