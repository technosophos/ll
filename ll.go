package main

import (
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"path/filepath"

	"github.com/Masterminds/goutils"
	"github.com/dustin/go-humanize"
)

var filenameWidth = 30
var maxWidth = 80

func main() {
	dir := "."
	if len(os.Args) > 1 {
		dir = os.Args[1]
	}
	if err := ll(dir, os.Stdout); err != nil {
		fmt.Fprintf(os.Stderr, "\ue231 %s\n", err)
		os.Exit(1)
	}
}

func ll(dir string, out io.Writer) error {
	fi, err := os.Stat(dir)
	if err != nil {
		return err
	}
	if !fi.IsDir() {
		printFI(fi, out)
		return nil
	}

	files, err := ioutil.ReadDir(dir)
	if err != nil {
		return err
	}

	printHeader(dir, files, out)
	for _, fi := range files {
		printFI(fi, out)
	}

	printFooter(dir, files, out)
	return nil
}

func printHeader(dir string, files []os.FileInfo, out io.Writer) {
	//fmt.Fprintln(out, strings.Repeat("=", maxWidth))
	fullpath, _ := filepath.Abs(dir)
	if plen := len(fullpath); plen > maxWidth {
		offset := maxWidth
		fullpath, _ = goutils.AbbreviateFull(fullpath, offset, maxWidth)
	}
	fmt.Fprintf(out, "\033[0;33m\uf07c\033[0;34m  %s\033[0m\n", fullpath)
}

func printFooter(dir string, files []os.FileInfo, out io.Writer) {
	numFiles := len(files)
	var sumFiles int64 = 0
	for _, fi := range files {
		sumFiles += fi.Size()
	}
	//fmt.Fprint(out, "\033[0;34m")
	fmt.Fprintf(out, "     \033[0;34m\uf00e  Total: %6d   Size: %10s\033[0m\n", numFiles, humanize.Bytes(uint64(sumFiles)))
}

func printFI(fi os.FileInfo, out io.Writer) {
	format := " %s   %-30s %8s     \uf017 %s\n"
	fsize := humanize.Bytes(uint64(fi.Size()))
	when := humanize.Time(fi.ModTime())
	fname, _ := goutils.Abbreviate(fi.Name(), filenameWidth)
	// when := fi.ModTime().Format(time.Stamp)
	fmt.Fprintf(out, format, icon(fi), fname, fsize, when)
}

// icon maps files to icons: https://github.com/ryanoasis/nerd-fonts
func icon(fi os.FileInfo) string {
	if fi.IsDir() {
		return "\033[0;33m\uf07b\033[0m" // closed folder
	}
	color := ""
	end := ""
	mode := fi.Mode()
	if mode&os.ModeSymlink != 0 {
		color = "\033[0;33m"
		end = "\033[0m"
		return color + "\uf0c1" + end
	} else if fi.Mode().Perm()&0100 != 0 {
		color = "\033[0;31m"
		end = "\033[0m"
	}

	// Some files get special treatment because of their names
	//ico := "\uf15b"
	ico := "\uf036"
	switch fi.Name() {
	case ".gitignore":
		ico = "\uf113"
	case "Dockerfile":
		ico = "\ue7b0"
	case "Makefile":
		ico = "\ue20f"
	case "acid.js":
		ico = "\uf0c3"
	case "glide.yaml", "Gopkg.toml":
		ico = "\uf1d8"
	case "package.json":
		ico = "\ue718"
	case "LICENSE", "license", "LICENSE.txt", "license.txt", "COPYING", "COPYING.txt":
		ico = "\uf071"
	case "README", "README.txt", "README.md":
		ico = "\uf06e"
	case ".DS_Store":
		ico = "\ue711"
	case ".bashrc", ".zshrc", ".profile":
		ico = "\uf120"
	default:
		ext := filepath.Ext(fi.Name())
		switch ext {
		case "":
			ico = "\uf15b"
		case ".sh", ".bash", ".zsh", ".ksh":
			ico = "\uf120"
		case ".lock":
			ico = "\uf023"
		case ".js":
			ico = "\ue781"
		case ".log":
			ico = "\uf18d"
		case ".yaml", ".yml":
			ico = "\uf19e"
		case ".json":
			ico = "\ue60b"
		case ".conf", "cfg":
			ico = "\uf085"
		case ".bin":
			ico = "\uf1b3"
		case ".err":
			ico = "\uf1e2"
		case ".ts":
			ico = "\ue628"
		case ".py", ".pyc", ".pyo":
			ico = "\ue73c"
		case ".md", ".markdown":
			ico = "\ue609"
		case ".rb":
			ico = "\ue739"
		case ".java":
			ico = "\ue738"
		case ".go":
			ico = "\ue626" // solid gopher
		case ".html", ".htm", ".xhtml":
			ico = "\uf1c9"
		case ".php", ".phar":
			ico = "\ue73d"
		case ".tpl", ".tmpl":
			ico = "\ue60e"
		case ".coffee":
			ico = "\ue61b"
		case ".tgz", ".gz", ".zip", ".bz2", ".bz":
			ico = "\uf1c6"
		case ".pdf":
			ico = "\uf1c1"
		case ".png", ".gif", ".jpg", ".jpeg", ".svg":
			ico = "\ue60d"
		case ".doc", ".docx", ".odt":
			ico = "\uf1c2"
		case ".ppt", ".pptx":
			ico = "\uf1c4"
		case ".xls", ".xlsx":
			ico = "\uf1c3"
		case ".pub", ".gpg", ".gpg~", ".kbx", ".kbx~":
			ico = "\uf084"
		case ".db", ".sqlite", ".sqlite3":
			ico = "\uf1c0"
		case ".mov", ".mp4":
			ico = "\uf008"
		case ".backup":
			ico = "\uf0fa"
		case ".vim":
			ico = "\ue7c5"
		case ".mk", ".make":
			ico = "\ue20f"
		case ".h":
			ico = "\uf0fd"
		case ".c":
			ico = "\ue61e"
		case ".cc", "c++":
			ico = "\ue61d"

		}
	}

	return color + ico + end
}
