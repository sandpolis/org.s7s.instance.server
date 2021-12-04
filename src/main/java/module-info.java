//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//
open module com.sandpolis.server.vanilla {

	requires com.sandpolis.core.clientserver;
	requires com.sandpolis.core.foundation;
	requires com.sandpolis.core.instance;
	requires com.sandpolis.core.server;
	requires com.sandpolis.core.serveragent;
	requires io.netty.buffer;
	requires io.netty.codec;
	requires io.netty.common;
	requires io.netty.handler;
	requires io.netty.transport;
	requires java.naming;
	requires org.slf4j;

	requires jdk.unsupported;
}
