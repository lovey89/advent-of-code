#!/usr/bin/env python3

import sys
import re
import operator

input_pattern = re.compile(r'\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.*)')
message_pattern = re.compile(r'Guard #(\d+) begins shift')

class LogEntry:
    def __init__(self, time_tuple, message):
        self.time_tuple = time_tuple
        self.message = message

    def get_minute_field(self):
        return self.time_tuple[4]

class GuardSleepingPattern:
    def __init__(self, guard_id):
        self.guard_id = guard_id
        self.sleep_pattern = [0 for x in range(60)]

    def update_sleep_pattern(self, start, end):
        for index in range(start, end):
            self.sleep_pattern[index] += 1

def read_file(file_name):
    log_entries = []

    with open(file_name) as f:
        for line in f:
            match = input_pattern.match(line)
            year, month, day, hour, minute, message = match.group(1, 2, 3, 4, 5, 6)
            log_entries.append(LogEntry((year, month, day, hour, int(minute)), message))

    return log_entries

if __name__ == '__main__':
    file_name = sys.argv[1]

    log_entries = read_file(file_name)
    log_entries.sort(key = lambda log_entry: log_entry.time_tuple)

    guard_sleep_patterns = {}

    for log_entry in log_entries:
        first_char = log_entry.message[0]
        if first_char == 'f':
            # Guard falls asleep
            fell_asleep = log_entry.get_minute_field()
        elif first_char == 'w':
            current_guard.update_sleep_pattern(fell_asleep, log_entry.get_minute_field())
        else:
            match = message_pattern.match(log_entry.message)
            guard_id = int(match.group(1))
            current_guard = guard_sleep_patterns.setdefault(guard_id, GuardSleepingPattern(guard_id))

    minute_most_slept_by_guard = []
    for guard_info in guard_sleep_patterns.values():
        minute, amount_slept = max(enumerate(guard_info.sleep_pattern), key=operator.itemgetter(1))
        minute_most_slept_by_guard.append((amount_slept, guard_info.guard_id, minute))

    _, guard_id, minute = max(minute_most_slept_by_guard)

    print(guard_id * minute)
