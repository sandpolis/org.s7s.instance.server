//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//
package com.sandpolis.server.vanilla;

import com.sandpolis.core.instance.Entrypoint;
import com.sandpolis.core.instance.Metatypes.InstanceFlavor;
import com.sandpolis.core.instance.Metatypes.InstanceType;
import com.sandpolis.core.instance.init.InstanceLoadEnvironment;
import com.sandpolis.core.instance.init.InstanceLoadPlugins;
import com.sandpolis.core.server.init.ServerFirstTimeSetup;
import com.sandpolis.core.server.init.ServerLoadListeners;
import com.sandpolis.core.server.init.ServerLoadStores;

public final class Main extends Entrypoint {

	private Main(String[] args) {
		super(Main.class, InstanceType.SERVER, InstanceFlavor.SERVER_VANILLA);

		register(new InstanceLoadEnvironment());
		register(new ServerLoadStores());
		register(new InstanceLoadPlugins());
		register(new ServerFirstTimeSetup());
		register(new ServerLoadListeners());

		start("Sandpolis Server", args);
	}

	public static void main(String[] args) {
		new Main(args);
	}
}
