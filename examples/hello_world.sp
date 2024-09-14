#pragma newdecls required
#pragma semicolon 1

public Plugin myinfo = {
	name = "Hello, world!",
	author = "Me",
	description = "A classic 'Hello, world!' example.",
	version = "0.1.0",
	url = "https::/example.com"
};

public void OnPluginStart() {
	LogMessage("Hello, world!");
	LogMessage("Hello, %s!", "World");
	int nWorlds = 427;
	LogMessage("Hello to %d world(s)!", nWorlds);
	LogMessage("Hello to a maximum of %d client(s)!", MaxClients);
}
