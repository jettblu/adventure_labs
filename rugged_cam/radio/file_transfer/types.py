from enum import Enum


class ResultSender(Enum):
    SUCCESS = 1
    FAIL = 2
    NO_FILES = 3
    FILE_NOT_FOUND = 4