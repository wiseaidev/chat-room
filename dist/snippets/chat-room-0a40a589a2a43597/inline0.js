export function __cargo_web_snippet_5454556031c2517bb4af26b52bd11c27dede1e0d(Module, $0, $1, $2, $3, $4, $5, $6, $7, $8) { $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);$5 = Module.STDWEB_PRIVATE.to_js($5);$6 = Module.STDWEB_PRIVATE.to_js($6);$7 = Module.STDWEB_PRIVATE.to_js($7);$8 = Module.STDWEB_PRIVATE.to_js($8);Module.STDWEB_PRIVATE.from_js($0, (function(){let pubnub=($1);let nickname=($2);let chat_callback=($3);let online_cb=($4);let offline_cb=($5);console.log("PubNub Chat Engine Ready");pubnub.subscribe({channels:[($6)]});pubnub.addListener({message:function(m){if(m.publisher !=nickname){let user=nickname.length>0?nickname:m.publisher;chat_callback(m.message,user);}}});console.log("The chat is connected!");pubnub.hereNow({channels:[($7)],includeState:true,}).then((response)=>{let occupants=response["channels"][($8)]["occupants"];let user=occupants[occupants.length-1];user=nickname.length>0?nickname:user["uuid"];let msg="User "+user+" is Online.";console.log(msg);online_cb(user);}).catch((error)=>{console.log(error)});console.log("pubnub connecting");return pubnub;})()); }