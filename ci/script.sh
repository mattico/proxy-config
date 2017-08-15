# This script takes care of testing your crate

set -ex

main() {
    cross build --target $TARGET
    cross build --target $TARGET --release
    cross build --target $TARGET --examples

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    # need to run tests serially, since we modify env vars
    cross test --target $TARGET -- --test-threads=1
    cross test --target $TARGET --release -- --test-threads=1

    cross run --target $TARGET
    cross run --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
