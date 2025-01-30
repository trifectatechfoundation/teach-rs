#! /bin/sh

show_help() {
    echo "usage: deploy.sh <path-to-teach-rs> --list" 
    echo "usage: deploy.sh <path-to-teach-rs> --to <target-path> --all"
    echo "usage: deploy.sh <path-to-teach-rs> --to <target-path> --select <track> <track> ..."
    exit 1
}

TEACHRS="$1"
OPERATION="$2"
TARGET="$3"
OPTION="$4"

if [ -z "$TEACHRS" ] || [ -z "$OPERATION" ]; then
    show_help
fi

if ! which npm > /dev/null; then
    echo "this script requires 'npm' to be installed"
    exit 1
fi

if ! which mdbook > /dev/null; then
    echo "this script requires 'mdbook' to be installed"
    exit 1
fi

if ! [ -d "$TEACHRS" ]; then
    echo "$TEACHRS: not found"
    exit 1
fi

if ! [ -d "$TEACHRS/modmod" ]; then
    echo "$TEACHRS does not appear to contain a master version of teach-rs"
    exit 1
fi

if ! which modmod > /dev/null; then
    echo "modmod not found! please run: cargo install --path \"$TEACHRS/modmod\""
    exit 1
fi

perform_deploy() {
    TARGET=$(realpath -q "$TARGET")

    for track in "$@"; do
	track="${track##*/}"
	track="${track%.track.toml}"
	echo "- $track"

	[ "$OPERATION" = --to ] || continue

	toml="$TEACHRS/content/$track.track.toml"

	if [ -d "$TEACHRS/instance/$track" ]; then
	    echo "[re-using $TEACHRS/instance/$track] "
	else
	    printf "[instantiating] "
	    modmod generate --output "$TEACHRS/instance/$track" --slide-url-base "/$track/" --clear "$toml"
	fi

	echo "[building the book]"
	mdbook build --dest-dir "$TARGET/$track" "$TEACHRS/instance/$track/book"

	(
	    echo "[bundling exercises]"
	    cd "$TEACHRS/instance/$track"
	    mkdir -p "$TARGET/$track"
	    zip --quiet -r "$TARGET/$track/exercises.zip" exercises
	)

	(
	    echo "[building the slides]"
	    cd "$TEACHRS/instance/$track/slides"
	    npm install
	    for slide in *.md; do
		slidenr="${slide%%-*}"
		npm run build-"$slidenr"
		mkdir -p "$TARGET/$track/slides"
		cp -r "dist/$slidenr"-* "$TARGET/$track/slides/$slidenr"
	    done
	)
    done
}

case "$OPERATION" in
    --list) 
        echo "available tracks:"
	perform_deploy "$TEACHRS/content"/*.track.toml
	exit 0;;

    --to) break;;

    *)  echo "expected '--to <dir>', got '$OPERATION'"
        show_help;;
esac

if [ -d "$TARGET" ]; then
    echo "$TARGET: already exist, refusing to mess up your file system"
    exit 1
fi

set -e

shift 4
mkdir -p "$TARGET"

case "$OPTION" in
    --all)
	if ! [ -z "$*" ]; then
	    echo "unexpected arguments: '$*' after '--all', abandoning"
	    show_help
	fi
        echo "deploying (this may take a while):"
	perform_deploy "$TEACHRS/content"/*.track.toml;;

    --select)
	if [ -z "$*" ]; then
	    echo "please select some tracks to deploy after '--select'"
	    show_help
	fi
        echo "deploying:"
	perform_deploy "$@";;

    *)  echo "expected '--all' or '--select <track>, got '$OPTION'"
        show_help;;
esac
