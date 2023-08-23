#include "quickjs.h"


void JS_FreeValue_Inline(JSContext *ctx, JSValue v){
    return JS_FreeValue(ctx,v);
};

void JS_FreeValueRT_Inline(JSRuntime *rt, JSValue v){
    return JS_FreeValueRT(rt,v);
};

void JS_DupValue_Inline(JSContext *ctx, JSValue v){
    JS_DupValue(ctx,v);
};

void JS_DupValueRT_Inline(JSRuntime *rt, JSValue v){
    JS_DupValueRT(rt,v);
};

