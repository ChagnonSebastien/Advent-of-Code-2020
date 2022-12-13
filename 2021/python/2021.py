import os
import time
from io import StringIO
from contextlib import redirect_stdout

if __name__ == '__main__':
    total_time = 0

    try:
        print(f'            Answer          Time')

        for day in range(1, 26):
            print(f'DAY {day:0>2}')
            os.chdir(f'Day{day:0>2}')

            for part in (1, 2):

                start_time = time.perf_counter()

                answer = StringIO()
                with redirect_stdout(answer):
                    exec(open(f'Part{part}.py').read())

                time_diff = time.perf_counter() - start_time
                total_time += time_diff
                print(f'  Part {part}    {answer.getvalue().strip():<16}{round(time_diff * 1000, 1)} milliseconds')

            os.chdir(f'..')

    except FileNotFoundError:
        print('  No data yet available')

    print()
    print(f'Total time: {round(total_time * 1000, 1)} milliseconds')


