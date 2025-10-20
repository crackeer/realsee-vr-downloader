"use client";

import React, { useState, useEffect } from "react";

import {
    Tabs,
} from "@arco-design/web-react";

import Download from './Download'
import Convert from './Convert'
const TabPane = Tabs.TabPane;
import { getPageData, updatePageData } from "@/store/page";
export default function VRPage() {
    const [activeTab, setActiveTab] = useState('');
    useEffect(() => {
        getPageData('activeTab').then((data) => {
            setActiveTab(data || 'download');
        });
    }, []);
    const changeActiveTab = (tab) => {
        setActiveTab(tab);
        updatePageData('activeTab', tab);
    }
    return (
        <div>
            <Tabs activeTab={activeTab} onChange={changeActiveTab} >
                <TabPane title="VR下载" key="download" destroyOnHide={false}>
                    <Download />
                </TabPane>
                <TabPane title="VR转换" key="convert">
                    <Convert />
                </TabPane>
            </Tabs>
        </div>
    );
};