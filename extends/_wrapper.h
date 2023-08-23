#include "quickjs.h"

void JS_FreeValue_Inline(JSContext *ctx, JSValue v);
void JS_FreeValueRT_Inline(JSRuntime *rt, JSValue v);
void JS_DupValue_Inline(JSContext *ctx, JSValue v);
void JS_DupValueRT_Inline(JSRuntime *rt, JSValue v);


int JS_VALUE_IS_NAN_Inline(JSValue v){ return JS_VALUE_IS_NAN(v); };//AWK CREATE[DONT REMOVE] 
int JS_IsNumber_Inline(JSValueConst v){ return JS_IsNumber(v); };//AWK CREATE[DONT REMOVE] 
int JS_IsBigFloat_Inline(JSValueConst v){ return JS_IsBigFloat(v); };//AWK CREATE[DONT REMOVE] 
int JS_IsBigDecimal_Inline(JSValueConst v){ return JS_IsBigDecimal(v); };//AWK CREATE[DONT REMOVE] 
int JS_IsBool_Inline(JSValueConst v){ return JS_IsBool(v); };//AWK CREATE[DONT REMOVE] 
int JS_IsNull_Inline(JSValueConst v){ return JS_IsNull(v); };//AWK CREATE[DONT REMOVE] 
int JS_IsUndefined_Inline(JSValueConst v){ return JS_IsUndefined(v); };//AWK CREATE[DONT REMOVE] 
int JS_IsException_Inline(JSValueConst v){ return JS_IsException(v); };//AWK CREATE[DONT REMOVE] 
int JS_IsUninitialized_Inline(JSValueConst v){ return JS_IsUninitialized(v); };//AWK CREATE[DONT REMOVE] 
int JS_IsString_Inline(JSValueConst v){ return JS_IsString(v); };//AWK CREATE[DONT REMOVE] 
int JS_IsSymbol_Inline(JSValueConst v){ return JS_IsSymbol(v); };//AWK CREATE[DONT REMOVE] 
int JS_IsObject_Inline(JSValueConst v){ return JS_IsObject(v); };//AWK CREATE[DONT REMOVE] 
