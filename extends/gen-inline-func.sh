#!/bin/bash

# awk exists?
which "awk" > /dev/null 2>&1
if [ $? -ne 0 ]; then
  echo "NOT FOUND EXE: awk"
  exit 1
fi


ROOT_DIR="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
WRAPPER_FILE="${ROOT_DIR}/_wrapper.h"
WRAPPER_BACKUP="${ROOT_DIR}/_wrapper.bak"
FILTER_FILE="${ROOT_DIR}/inline.list"


if [ ! -f "${WRAPPER_FILE}" ]; then
  echo "NOT FOUND WRAPPER_FILE:${WRAPPER_FILE}"
  exit 1
fi

if [ ! -f "${FILTER_FILE}" ]; then
  echo "NOT FOUND FILTER_FILE:${FILTER_FILE}"
  exit 1
fi

echo "============================================"
echo "Convert Inline Function To Rust Function"
echo "WRAPPER_FILE: ${WRAPPER_FILE}"
echo "FILTER_FILE: ${FILTER_FILE}"
echo "============================================"


# clean lines
cat ${WRAPPER_FILE} > ${WRAPPER_BACKUP}
grep -v -e 'AWK CREATE' -e 'DONT REMOVE' $WRAPPER_BACKUP > ${WRAPPER_FILE}

# append line
cat ${WRAPPER_FILE} > ${WRAPPER_BACKUP}
awk -v br="//AWK CREATE[DONT REMOVE]" 'BEGIN{FS=OFS=":"} { printf "%s %s_%s%s{ return %s%s; };%s \r\n", $1,$2,"Inline",$3,$2,$4,br }' ${FILTER_FILE} >> ${WRAPPER_BACKUP}
cat ${WRAPPER_BACKUP} > ${WRAPPER_FILE}

# remove temp
rm ${WRAPPER_BACKUP}