package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path"
	"runtime"
	"strings"
)

func main() {
	if len(os.Args) < 2 || !strings.HasSuffix(os.Args[1], ".clj") {
		fmt.Printf("need input clj file\n")
		return
	}
	file, err := os.OpenFile(os.Args[1], os.O_RDONLY, 0)
	if err != nil {
		fmt.Println(err)
		if os.IsNotExist(err) {
			fmt.Printf("input file %s not exist\n", os.Args[1])
		}
		return
	}
	dir := path.Dir(strings.ReplaceAll(os.Args[1], "\\", "/"))

	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)
	currentLine := ""
	for fileScanner.Scan() {
		currentLine = strings.TrimSpace(fileScanner.Text())
		if strings.HasPrefix(currentLine, ";") &&
			strings.Contains(currentLine, "clojure") {
			currentLine = strings.Replace(currentLine, ";", "", 1)
			currentLine = strings.ReplaceAll(currentLine, "\\", "/")
			//currentLine = strings.Replace(currentLine, ":deps",
			//	fmt.Sprintf(":paths [\".\"] :deps"), 1)
			fmt.Printf("run command: %s ", currentLine)
			break
		}
	}
	_ = file.Close()
	if currentLine == "" {
		fmt.Printf("no command need to run\n")
		return
	}
	command := currentLine
	var cmd *exec.Cmd
	if runtime.GOOS == "windows" {
		cmd = exec.Command("powershell", "-NoProfile", command)
	} else if runtime.GOOS == "darwin" || runtime.GOOS == "linux" {
		cmd = exec.Command("bash", "-c", command)
	}
	fmt.Printf("at dir %s\n", dir)
	stdout, err := cmd.StdoutPipe()
	//out, err := cmd.CombinedOutput() ;get output instantly
	//fmt.Printf("result: %s\n", out)
	cmd.Stderr = os.Stderr
	cmd.Dir = dir
	err = cmd.Start()
	if err != nil {
		_ = fmt.Errorf("run command error: %v", err)
		return
	}
	reader := bufio.NewReader(stdout)
	for {
		line, err2 := reader.ReadString('\n')
		if err2 != nil || io.EOF == err2 {
			break
		}
		fmt.Print(line)
	}
	err = cmd.Wait()
	fmt.Printf("Press Enter to leave\n")
	_, _ = fmt.Scanln()
}
