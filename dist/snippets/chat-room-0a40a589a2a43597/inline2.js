export function __cargo_web_snippet_abcc153075af5d9754083aeeaa15d8371da8d185(Module, $0, $1, $2) { $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){let pubnub=new PubNub({publishKey:($1),subscribeKey:($2),userId:"user_"+Math.random().toString(36).substring(20,1)});console.log("PubNub instance created!");return pubnub;})()); }