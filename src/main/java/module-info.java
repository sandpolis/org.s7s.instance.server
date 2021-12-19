//============================================================================//
//                                                                            //
//            Copyright © 2015 - 2022 Sandpolis Software Foundation           //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPLv2. //
//                                                                            //
//============================================================================//
open module org.s7s.instance.server.java {

	requires org.s7s.core.clientserver;
	requires org.s7s.core.foundation;
	requires org.s7s.core.instance;
	requires org.s7s.core.server;
	requires org.s7s.core.serveragent;
	requires io.netty.buffer;
	requires io.netty.codec;
	requires io.netty.common;
	requires io.netty.handler;
	requires io.netty.transport;
	requires java.naming;
	requires org.slf4j;

	requires jdk.unsupported;
}
