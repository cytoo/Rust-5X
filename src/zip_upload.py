import subprocess
import sys
import os

def zip_the_files():
    rom_name = sys.argv[1]
    os.chdir("..")
    cmd = subprocess.Popen(f"cd ErfanGSIs/output;sudo zip -r {rom_name}-GSI-AB.zip *-AB-*.img;sudo zip -r {rom_name}-GSI-Aonly.zip *-Aonly-*.img",shell=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    stdout,stderr = cmd.communicate()
    output = str(stdout.decode() + stderr.decode())
    print(output)

def upload_the_files():
    print(os.getcwd())
    up = subprocess.Popen(f"cp -r ErfanGSIs/output/*-AB-*.txt bruh.txt;touch ab.txt;touch aonly.txt;./transfer wet ErfanGSIs/output/*-AB.zip | grep Download >> ab.txt;./transfer wet ErfanGSIs/output/*-Aonly.zip | grep Download >> aonly.txt",shell=True,stderr=subprocess.PIPE,stdout=subprocess.PIPE)
    stdout,stderr = up.communicate()
    output = str(stdout.decode() + stderr.decode())
    print(output)

if __name__ == "__main__":
    zip_the_files()
    upload_the_files()
    print("process completed")