#!/bin/bash
cp -r ../handson* .

rm -rf handson*/.git//

cp -r ../../quad .

git add . 
git commit -m "godo"
git push

