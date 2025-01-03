from enum import Enum

class ECardStateEntries(Enum):
    UID      = 0
    LOC_NAME = 1
    LOC_NUM  = 2
    P1V_UID  = 3
    P1V_LOC  = 4
    P2V_UID  = 5
    P2V_LOC  = 6
    CARD_STATE_MAX_SIZE = 7

class ESupportedPlayers(Enum):
    PLAYER_1 = 0
    PLAYER_2 = 1
    MAX_SUPPORTED_PLAYERS = 2
