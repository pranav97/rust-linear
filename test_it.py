import re
import numpy as np
import sys
import subprocess
import csv


class MatrixTester(object):
    def __init__(self):
        self.test_num = 0

    def start(self):
        pass

    def reset_counter(self):
        self.test_num = 0

    def print_matrix_to_file(self):
        num_rows = int(input("Input the number of rows: "))
        num_cols = int(input("Input the number of cols: "))
        file_name = input("Input the filename: ")
        mat = np.random.randint(1, 100, size=(num_rows, num_cols))

        with open(file_name, "w") as f:
            f.write(self.get_input_text(num_rows, num_cols, mat))

    def parse_output(self, str_out, perf_test=False):
        micro_seconds = None

        target_text = None
        target_text = str_out.find('Product is:')
        if target_text == -1 :
            sys.exit(1)
        target_text += len('Product is:\n')
        
        time_taken = re.search(
            "Time taken by multiplication: (\d+) microseconds", str_out)
        if time_taken is not None:
            micro_seconds = int(time_taken.group(1))

        if not perf_test:
            rest = str_out[target_text:]
            rows = re.split('\n', rest.strip())
            dda = []
            for r in rows:
                dda.append([int(i) for i in re.split('\t', r.strip())])
            ret = np.array(dda)
            return ret
        else:
            return micro_seconds


    def print_report(self, failed):
        print("\nReport:", end=" ")
        print("Passed {}/{} {} tests".format(self.test_num,
                                                 self.test_num, "Multiplication"))
        for i in failed:
            print(i)

    def get_input_text(self, row, col, mat):
        input_text = ""
        input_text += "{}\n".format(row)
        input_text += "{}\n".format(col)
        for r in mat:
            for c in r:
                input_text += '{}\n'.format(c)
        return input_text

    def mult_test(
            self, row1, col1, row2, col2,
            perf_test=False, single_thread=False):
        mat = np.random.randint(1, 100, size=(row1, col1))
        mat2 = np.random.randint(1, 100, size=(row2, col2))

        prod = mat.dot(mat2)
        input_text = self.get_input_text(row1, col1, mat)
        input_text += self.get_input_text(row2, col2, mat2)

        cmd = ["target/debug/multi_thread"]
        if single_thread:
            cmd[-1] = 'target/debug/single_thread'
        proc = subprocess.Popen(cmd,
                                stdin=subprocess.PIPE,
                                stdout=subprocess.PIPE
                                )
        (out, ret) = proc.communicate(str.encode(input_text))
        
        str_out = out.decode("utf-8")
        result = self.parse_output(
            str_out, perf_test=perf_test)
        if perf_test:
            return result
        if np.array_equal(result, prod):
            return "PASS"
        return "FAIL\n Expected {}\n Got {}\n".format(prod.__repr__(), str_out)

    def correctness_mult_tests(self):
        print("\n\nRunning Multiplication Tests")
        failed = []
        min_test = 10
        max_test = 100
        inc = 10
        for a in range(min_test, max_test, inc):
            for b in range(a, max_test, inc):
                for c in range(min_test, max_test, inc):
                    # print("testing {} x {} mult {} x {}".format(a, b, b, c))
                    ret = self.mult_test(a, b, b, c)
                    if ret == "PASS":
                        if self.test_num % 50 == 0:
                            print("")
                        print(".", end=" ", flush=True)
                    else:
                        print("x", end=" ", flush=True)
                        failed.append("test {} row {} col {}: {}".format(
                            self.test_num, a, c, ret))
                    self.test_num += 1
        self.print_report(failed)

    def performance_mult(self):
        print("\n\nMult tests")
        num_rows = [500,
                    1000,
                    1500,
                    2000
                    ]
        f = open('speedup.csv', 'w')
        with f:
            fnames = ['num_rows','single', 'multi', 'speedup']
            writer = csv.DictWriter(f, fieldnames=fnames)    
            writer.writeheader()
            for i in range(len(num_rows)):
                print("Test {}: {} rows and cols".format(i, num_rows[i]))
                ret = self.mult_test(
                    num_rows[i], num_rows[i], num_rows[i], num_rows[i],
                    perf_test=True, single_thread=True)
                print("Single Thread: {} microseconds".format(ret))
                speed = self.mult_test(
                    num_rows[i], num_rows[i], num_rows[i], num_rows[i],
                    perf_test=True)
                print("Multi  Thread: {} microseconds".format(speed))
                up = float(ret/speed)
                print("* {} x speedup".format(up))
                writer.writerow({'num_rows':num_rows[i], 'single' : ret, 'multi': speed, 'speedup': up})


def run_performance_tests():
    print("\n\nRunning Performance tests")
    MatrixTester().performance_mult()


def run_correctness_tests():
    MatrixTester().correctness_mult_tests()


def run_all_tests():
    run_correctness_tests()
    run_performance_tests()


def main():
    if len(sys.argv) > 1:
        if sys.argv[1] == "-p":
            print_matrix_to_file()
    else:
        run_all_tests()


def print_matrix_to_file():
    MatrixTester().print_matrix_to_file()


if __name__ == "__main__":

    main()

