package main

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
	"path"
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
	//fmt.Printf("dir is %s", dir)

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
	cmd := exec.Command("powershell", "-NoProfile", command)
	cmd.Dir = dir
	fmt.Printf("at dir %s\n", dir)
	out, err := cmd.CombinedOutput()
	if err != nil {
		_ = fmt.Errorf("run command error: %v", err)
		return
	}
	fmt.Printf("result: %s\n", out)
	fmt.Printf("Press Enter to leave\n")
	_, _ = fmt.Scanln()
}
