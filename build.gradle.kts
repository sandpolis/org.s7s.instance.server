//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

import com.bmuschko.gradle.docker.tasks.container.*
import com.bmuschko.gradle.docker.tasks.image.*

plugins {
	id("java-library")
	id("sandpolis-java")
	id("sandpolis-module")
	id("sandpolis-publish")
	id("sandpolis-soi")
	id("com.bmuschko.docker-remote-api") version "6.6.0"
}

dependencies {
	testImplementation("org.junit.jupiter:junit-jupiter-api:5.6.1")
	testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.6.1")

	implementation(project(":module:com.sandpolis.core.server"))

	// https://github.com/netty/netty
	implementation("io.netty:netty-codec:4.1.48.Final")
	implementation("io.netty:netty-common:4.1.59.Final")
	implementation("io.netty:netty-handler:4.1.48.Final")
	implementation("io.netty:netty-transport:4.1.48.Final")

	implementation(project(":com.sandpolis.agent.installer:go"))
	implementation(project(":com.sandpolis.agent.installer:jar"))
	implementation(project(":com.sandpolis.agent.installer:py"))
}

task<Sync>("assembleLib") {
	dependsOn(tasks.named("jar"))
	from(configurations.runtimeClasspath)
	from(tasks.named("jar"))
	into("${buildDir}/lib")
}

task<DockerBuildImage>("buildImage") {
	dependsOn(tasks.named("assembleLib"))
	inputDir.set(file("."))
	images.add("sandpolis/server/vanilla:${project.version}")
	images.add("sandpolis/server/vanilla:latest")
}

task<Exec>("runImage") {
	dependsOn(tasks.named("buildImage"))
	commandLine("docker", "run", "-p", "8768:8768", "--rm", "sandpolis/server/vanilla:latest")
}
