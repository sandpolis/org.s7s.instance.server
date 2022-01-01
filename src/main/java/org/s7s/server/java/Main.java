//============================================================================//
//                                                                            //
//            Copyright © 2015 - 2022 Sandpolis Software Foundation           //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPLv2. //
//                                                                            //
//============================================================================//
package org.s7s.instance.server.java;

import org.s7s.core.instance.Entrypoint;
import org.s7s.core.foundation.Instance.InstanceFlavor;
import org.s7s.core.foundation.Instance.InstanceType;
import org.s7s.core.instance.init.InstanceLoadPlugins;
import org.s7s.core.server.init.ServerFirstTimeSetup;
import org.s7s.core.server.init.ServerLoadListeners;
import org.s7s.core.server.init.ServerLoadStores;

public final class Main extends Entrypoint {

	private Main(String[] args) {
		super(Main.class, InstanceType.SERVER, InstanceFlavor.SERVER_JAVA);

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
