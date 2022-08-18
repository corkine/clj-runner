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

func run() {
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
	all := strings.ReplaceAll(os.Args[1], "\\", "/")
	dir := path.Dir(all)
	filename := path.Base(all)

	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)
	currentLine := ""
	for fileScanner.Scan() {
		currentLine = strings.TrimSpace(fileScanner.Text())
		if strings.HasPrefix(currentLine, "#!/usr/bin/env bb") {
			currentLine = fmt.Sprintf("bb %s", filename)
			break
		}
		if strings.HasPrefix(currentLine, ";") &&
			strings.Contains(currentLine, "clojure") {
			currentLine = strings.Replace(currentLine, ";", "", 1)
			currentLine = strings.ReplaceAll(currentLine, "\\", "/")
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
		command = strings.ReplaceAll(command, "\"", "\"\"")
		fmt.Printf("run command: %s at dir %s\n", command, dir)
		cmd = exec.Command("powershell", "-NoProfile", command)
	} else if runtime.GOOS == "darwin" || runtime.GOOS == "linux" {
		fmt.Printf("run command: %s at dir %s\n", command, dir)
		cmd = exec.Command("bash", "-c", command)
	}
	stdout, err := cmd.StdoutPipe()
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
}

func main() {
	run()
	fmt.Printf("Press Enter to leave\n")
	_, _ = fmt.Scanln()
}
