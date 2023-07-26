#!/bin/bash

# If that wierd bug occours fix it
if [ -d "dist/frontend-source" ]; then
    rm dist/index.html && mv dist/frontend-source/index.html dist/ && rm -r dist/frontend-source/
fi
