//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

plugins {
	id("java-library")
	id("application")
	id("com.sandpolis.build.module")
	id("com.sandpolis.build.instance")
	id("com.sandpolis.build.publish")
}

application {
	mainModule.set("com.sandpolis.server.vanilla")
	mainClass.set("com.sandpolis.server.vanilla.Main")
	applicationDefaultJvmArgs = listOf("--enable-native-access=com.sandpolis.core.foreign")
}

tasks.named<JavaExec>("run") {
	environment.put("S7S_DEVELOPMENT_MODE", "true")
	environment.put("S7S_LOG_LEVELS", "io.netty=WARN,java.util.prefs=OFF,com.sandpolis=TRACE")
}

dependencies {
	testImplementation("org.junit.jupiter:junit-jupiter-api:5.8.2")
	testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.6.1")

	if (project.getParent() == null) {
		implementation("com.sandpolis:core.server:+")
		implementation("com.sandpolis:core.net:+")
		implementation("com.sandpolis:core.instance:+")
	} else {
		implementation(project(":module:com.sandpolis.core.server"))
		implementation(project(":module:com.sandpolis.core.net"))
		implementation(project(":module:com.sandpolis.core.instance"))
	}
}

if (System.getenv("S7S_BUILD_DISABLE_SERVER_PLUGINS") != "1") {
	val syncPlugins by tasks.creating(Copy::class) {
		into("build/plugin")

		project(":plugin").subprojects {
			afterEvaluate {
				tasks.findByName("pluginArchive")?.let { pluginArchiveTask ->
					from(pluginArchiveTask)
				}
			}
		}
	}

	afterEvaluate {
		tasks.findByName("run")?.dependsOn(syncPlugins)
	}
}
