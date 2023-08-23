#!/bin/bash

# awk exists?
which "awk" > /dev/null 2>&1
if [ $? -ne 0 ]; then
  echo "NOT FOUND EXE: awk"
  exit 1
fi


ROOT_DIR="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
HEADER_FILE="${ROOT_DIR}/../quickjs/quickjs.h"
OUTPUT_FILE="${ROOT_DIR}/inline.list"
OUTPUT_BACK="${ROOT_DIR}/inline.bak"
OUTPUT_BACK_TMP="${ROOT_DIR}/inline-tmp.bak"

if [ ! -f "${HEADER_FILE}" ]; then
  echo "NOT FOUND SOURCE_FILE:${HEADER_FILE}"
  exit 1
fi

echo "============================================"
echo "Filter Inline Function To inline.list File"
echo "HEADER_FILE: ${HEADER_FILE}"
echo "OUTPUT_FILE: ${OUTPUT_FILE}"
echo "============================================"

# write export function
awk '/static inline JS_BOOL/{print}' ${HEADER_FILE} > ${OUTPUT_BACK}

# remove JSContext *ctx
sed -e '/JSContext/d' ${OUTPUT_BACK} > ${OUTPUT_BACK_TMP}
cat ${OUTPUT_BACK_TMP} > ${OUTPUT_BACK}

# remove static line
sed -e 's/static inline JS_BOOL /int:/g' ${OUTPUT_BACK} > ${OUTPUT_BACK_TMP}
cat ${OUTPUT_BACK_TMP} > ${OUTPUT_BACK}

# merge '(' ,')'
sed -e 's/(/:(/g' ${OUTPUT_BACK} > ${OUTPUT_BACK_TMP}
cat ${OUTPUT_BACK_TMP} > ${OUTPUT_BACK}
sed -e 's/)/):(v)/g' ${OUTPUT_BACK} > ${OUTPUT_BACK_TMP}
cat ${OUTPUT_BACK_TMP}|uniq > ${OUTPUT_FILE}



# remove temp
rm ${OUTPUT_BACK}
rm ${OUTPUT_BACK_TMP}

