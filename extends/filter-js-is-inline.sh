#!/bin/bash

# awk exists?
which "awk" > /dev/null 2>&1
if [ $? -ne 0 ]; then
  echo "NOT FOUND EXE: awk"
  exit 1
fi


ROOT_DIR="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
SOURCE_FILE="${ROOT_DIR}/../quickjs/quickjs.h"
OUTPUT_FILE="${ROOT_DIR}/inline.list"
SOURCE_BACK="${ROOT_DIR}/inline.bak"
SOURCE_BACK_TMP="${ROOT_DIR}/inline-tmp.bak"

if [ ! -f "${SOURCE_FILE}" ]; then
  echo "NOT FOUND SOURCE_FILE:${SOURCE_FILE}"
  exit 1
fi

echo "============================================"
echo "Filter Inline Function To inline.list File"
echo "SOURCE_FILE: ${SOURCE_FILE}"
echo "OUTPUT_FILE: ${OUTPUT_FILE}"
echo "============================================"

# write export function
awk '/static inline JS_BOOL/{print}' ${SOURCE_FILE} > ${SOURCE_BACK}

# remove JSContext *ctx
sed -e '/JSContext/d' ${SOURCE_BACK} > ${SOURCE_BACK_TMP}
cat ${SOURCE_BACK_TMP} > ${SOURCE_BACK}

# remove static line
sed -e 's/static inline JS_BOOL /int:/g' ${SOURCE_BACK} > ${SOURCE_BACK_TMP}
cat ${SOURCE_BACK_TMP} > ${SOURCE_BACK}

# merge '(' ,')'
sed -e 's/(/:(/g' ${SOURCE_BACK} > ${SOURCE_BACK_TMP}
cat ${SOURCE_BACK_TMP} > ${SOURCE_BACK}
sed -e 's/)/):(v)/g' ${SOURCE_BACK} > ${SOURCE_BACK_TMP}
cat ${SOURCE_BACK_TMP}|uniq > ${OUTPUT_FILE}

# remove temp
rm ${SOURCE_BACK}
rm ${SOURCE_BACK_TMP}

