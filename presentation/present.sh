#!/bin/bash

# CNCF/KubeCon Slidev Presentation Helper
# Quick commands for your presentation

set -e

# Colors for output
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "╔════════════════════════════════════════════════════╗"
echo "║  CNCF/KubeCon Slidev Presentation                 ║"
echo "║  KubeCon + CloudNativeCon NA 2025                 ║"
echo "╚════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Check if we're in the right directory
if [ ! -f "slides.md" ]; then
    echo -e "${YELLOW}Warning: slides.md not found. Running from presentation directory...${NC}"
    cd "$(dirname "$0")"
fi

# Function to display help
show_help() {
    echo -e "${GREEN}Available commands:${NC}"
    echo ""
    echo "  ./present.sh dev       - Start development server"
    echo "  ./present.sh build     - Build for production"
    echo "  ./present.sh export    - Export to PDF"
    echo "  ./present.sh install   - Install dependencies"
    echo "  ./present.sh preview   - Preview production build"
    echo "  ./present.sh help      - Show this help"
    echo ""
}

# Parse command
case "$1" in
    dev)
        echo -e "${GREEN}Starting development server...${NC}"
        echo -e "${YELLOW}Tip: Press 'O' in the browser to open presenter mode${NC}"
        npm run dev
        ;;
    build)
        echo -e "${GREEN}Building for production...${NC}"
        npm run build
        echo -e "${GREEN}✓ Build complete! Output in dist/${NC}"
        ;;
    export)
        echo -e "${GREEN}Exporting to PDF...${NC}"
        npm run export
        echo -e "${GREEN}✓ Export complete!${NC}"
        ;;
    install)
        echo -e "${GREEN}Installing dependencies...${NC}"
        npm install
        echo -e "${GREEN}✓ Installation complete!${NC}"
        ;;
    preview)
        echo -e "${GREEN}Previewing production build...${NC}"
        if [ -d "dist" ]; then
            npx serve dist
        else
            echo -e "${YELLOW}No dist/ folder found. Run './present.sh build' first.${NC}"
        fi
        ;;
    help|--help|-h|"")
        show_help
        ;;
    *)
        echo -e "${YELLOW}Unknown command: $1${NC}"
        echo ""
        show_help
        exit 1
        ;;
esac
